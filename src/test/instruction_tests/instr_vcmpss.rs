use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 194, 248, 102], OperandSize::Dword)
}

#[test]
fn vcmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 194, 50, 121], OperandSize::Dword)
}

#[test]
fn vcmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 194, 250, 38], OperandSize::Qword)
}

#[test]
fn vcmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1792351652, Some(OperandSize::Dword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 194, 4, 125, 164, 29, 213, 106, 15], OperandSize::Qword)
}

#[test]
fn vcmpss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 102, 25, 194, 211, 6], OperandSize::Dword)
}

#[test]
fn vcmpss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 502886298, Some(OperandSize::Dword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 78, 13, 194, 60, 125, 154, 111, 249, 29, 115], OperandSize::Dword)
}

#[test]
fn vcmpss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM30)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 86, 31, 194, 230, 93], OperandSize::Qword)
}

#[test]
fn vcmpss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1218947724, Some(OperandSize::Dword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 62, 3, 194, 140, 135, 140, 170, 167, 72, 81], OperandSize::Qword)
}

