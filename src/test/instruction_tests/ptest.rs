use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 246], OperandSize::Dword)
}

fn ptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 171074226, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 182, 178, 98, 50, 10], OperandSize::Dword)
}

fn ptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 194], OperandSize::Qword)
}

fn ptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1684546078, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 36, 125, 30, 34, 104, 100], OperandSize::Qword)
}

