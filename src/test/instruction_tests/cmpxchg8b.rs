use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmpxchg8b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(Memory(20612, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 14, 132, 80], OperandSize::Word)
}

fn cmpxchg8b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 12, 190], OperandSize::Dword)
}

fn cmpxchg8b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG8B, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 15], OperandSize::Qword)
}

