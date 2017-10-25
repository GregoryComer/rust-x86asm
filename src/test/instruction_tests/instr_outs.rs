use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn outs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(SI, 9708, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Word)
}

#[test]
fn outs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Dword)
}

#[test]
fn outs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 862887558, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Qword)
}

#[test]
fn outs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Word)
}

#[test]
fn outs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(ESI, 1521246946, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Dword)
}

#[test]
fn outs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RBX, 40913293, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Qword)
}

#[test]
fn outs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Word)
}

#[test]
fn outs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Dword)
}

#[test]
fn outs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 934959486, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Qword)
}

