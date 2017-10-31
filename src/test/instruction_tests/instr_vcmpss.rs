use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 194, 197, 86], OperandSize::Dword)
}

#[test]
fn vcmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 194, 52, 119, 76], OperandSize::Dword)
}

#[test]
fn vcmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 194, 241, 22], OperandSize::Qword)
}

#[test]
fn vcmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 194, 23, 11], OperandSize::Qword)
}

#[test]
fn vcmpss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 94, 25, 194, 245, 62], OperandSize::Dword)
}

#[test]
fn vcmpss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 993803674, Some(OperandSize::Dword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 78, 15, 194, 36, 197, 154, 61, 60, 59, 86], OperandSize::Dword)
}

#[test]
fn vcmpss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM26)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 22, 27, 194, 250, 16], OperandSize::Qword)
}

#[test]
fn vcmpss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 70, 10, 194, 44, 94, 67], OperandSize::Qword)
}

