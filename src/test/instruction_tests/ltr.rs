use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ltr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 221], OperandSize::Word)
}

fn ltr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 25], OperandSize::Word)
}

fn ltr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 222], OperandSize::Dword)
}

fn ltr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1805522600, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28, 117, 168, 22, 158, 107], OperandSize::Dword)
}

fn ltr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 219], OperandSize::Qword)
}

fn ltr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LTR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 28, 186], OperandSize::Qword)
}

