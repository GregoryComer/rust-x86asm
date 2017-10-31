use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 930161150, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 136, 254, 33, 113, 55], OperandSize::Dword)
}

#[test]
fn movntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1438462419, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 140, 187, 211, 49, 189, 85], OperandSize::Qword)
}

