use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 44, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 122, 44], OperandSize::Word)
}

fn fstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 60, 194], OperandSize::Dword)
}

fn fstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1539857863, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 60, 157, 199, 93, 200, 91], OperandSize::Qword)
}

