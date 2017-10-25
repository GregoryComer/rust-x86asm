use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xlat_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XLAT, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 29, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[215], OperandSize::Word)
}

fn xlat_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XLAT, operand1: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[215], OperandSize::Dword)
}

fn xlat_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XLAT, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 227950787, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[215], OperandSize::Qword)
}

