use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn verr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 230], OperandSize::Word)
}

fn verr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 32], OperandSize::Word)
}

fn verr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 228], OperandSize::Dword)
}

fn verr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 453550428, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 164, 201, 92, 161, 8, 27], OperandSize::Dword)
}

fn verr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 226], OperandSize::Qword)
}

fn verr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectDisplaced(RSI, 1269747612, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 166, 156, 207, 174, 75], OperandSize::Qword)
}

