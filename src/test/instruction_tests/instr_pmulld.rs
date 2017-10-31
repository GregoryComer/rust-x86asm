use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 197], OperandSize::Dword)
}

#[test]
fn pmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1493024043, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 158, 43, 189, 253, 88], OperandSize::Dword)
}

#[test]
fn pmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 207], OperandSize::Qword)
}

#[test]
fn pmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 13929448, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 129, 232, 139, 212, 0], OperandSize::Qword)
}

