use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 197, 94], OperandSize::Dword)
}

#[test]
fn blendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 882677593, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 175, 89, 151, 156, 52, 56], OperandSize::Dword)
}

#[test]
fn blendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 242, 59], OperandSize::Qword)
}

#[test]
fn blendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 127036767, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 159, 95, 109, 146, 7, 65], OperandSize::Qword)
}

