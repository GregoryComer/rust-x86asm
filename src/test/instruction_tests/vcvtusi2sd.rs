use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtusi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 95, 8, 123, 250], OperandSize::Dword)
}

fn vcvtusi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 1125585787, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 119, 8, 123, 174, 123, 19, 23, 67], OperandSize::Dword)
}

fn vcvtusi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 7, 8, 123, 238], OperandSize::Qword)
}

fn vcvtusi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 533333361, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 15, 8, 123, 132, 248, 113, 5, 202, 31], OperandSize::Qword)
}

fn vcvtusi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 247, 24, 123, 203], OperandSize::Qword)
}

fn vcvtusi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 115704362, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 223, 8, 123, 158, 42, 130, 229, 6], OperandSize::Qword)
}

