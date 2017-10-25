use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn prefetcht1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 18072, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 145, 152, 70], OperandSize::Word)
}

fn prefetcht1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 20, 135], OperandSize::Dword)
}

fn prefetcht1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT1, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 266782639, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 148, 113, 175, 199, 230, 15], OperandSize::Qword)
}

