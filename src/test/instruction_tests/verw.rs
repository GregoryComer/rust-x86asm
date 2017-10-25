use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn verw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 235], OperandSize::Word)
}

fn verw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 24650, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 171, 74, 96], OperandSize::Word)
}

fn verw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 238], OperandSize::Dword)
}

fn verw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledDisplaced(EAX, Two, 676135125, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 44, 69, 213, 0, 77, 40], OperandSize::Dword)
}

fn verw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 235], OperandSize::Qword)
}

fn verw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERW, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 941012458, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 172, 246, 234, 181, 22, 56], OperandSize::Qword)
}

