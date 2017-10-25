use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bound_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 67, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 88, 67], OperandSize::Word)
}

fn bound_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(EDI, 302082235, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 183, 187, 104, 1, 18], OperandSize::Dword)
}

fn bound_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 79, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 98, 82, 79], OperandSize::Word)
}

fn bound_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BOUND, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 44, 202], OperandSize::Dword)
}

