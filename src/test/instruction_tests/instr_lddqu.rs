use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 60, 136], OperandSize::Dword)
}

#[test]
fn lddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDDQU, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 755455161, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 240, 128, 185, 84, 7, 45], OperandSize::Qword)
}

