use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fisubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 44], OperandSize::Word)
}

fn fisubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 43], OperandSize::Dword)
}

fn fisubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(RDI, 535647013, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 175, 37, 83, 237, 31], OperandSize::Qword)
}

fn fisubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(SI, 50, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 108, 50], OperandSize::Word)
}

fn fisubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 317056661, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 172, 250, 149, 230, 229, 18], OperandSize::Dword)
}

fn fisubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 566274415, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 44, 245, 111, 169, 192, 33], OperandSize::Qword)
}

