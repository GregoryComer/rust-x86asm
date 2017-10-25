use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fucomi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 233], OperandSize::Word)
}

fn fucomi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 234], OperandSize::Dword)
}

fn fucomi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 239], OperandSize::Qword)
}

