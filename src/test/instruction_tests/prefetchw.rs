use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn prefetchw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 12, 64], OperandSize::Dword)
}

fn prefetchw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1938408528, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 12, 197, 80, 196, 137, 115], OperandSize::Qword)
}

