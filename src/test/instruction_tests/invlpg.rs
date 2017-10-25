use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn invlpg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(Indirect(BX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 63], OperandSize::Word)
}

fn invlpg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1209325506, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 60, 245, 194, 215, 20, 72], OperandSize::Dword)
}

fn invlpg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INVLPG, operand1: Some(IndirectDisplaced(RAX, 1453276793, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 184, 121, 62, 159, 86], OperandSize::Qword)
}

