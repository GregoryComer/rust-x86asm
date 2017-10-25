use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 194, 246, 120], OperandSize::Dword)
}

#[test]
fn vcmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 26200979, Some(OperandSize::Dword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 194, 184, 147, 203, 143, 1, 54], OperandSize::Dword)
}

#[test]
fn vcmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 194, 212, 11], OperandSize::Qword)
}

#[test]
fn vcmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 194, 44, 71, 89], OperandSize::Qword)
}

#[test]
fn vcmpss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 86, 30, 194, 210, 28], OperandSize::Dword)
}

#[test]
fn vcmpss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 102, 10, 194, 49, 116], OperandSize::Dword)
}

#[test]
fn vcmpss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM26)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 126, 25, 194, 250, 108], OperandSize::Qword)
}

#[test]
fn vcmpss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSS, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 94, 12, 194, 44, 250, 35], OperandSize::Qword)
}

