use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fxsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 22, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 64, 22], OperandSize::Word)
}

fn fxsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectDisplaced(EDI, 1132793660, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 135, 60, 15, 133, 67], OperandSize::Dword)
}

fn fxsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1066948362, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 132, 250, 10, 87, 152, 63], OperandSize::Qword)
}

