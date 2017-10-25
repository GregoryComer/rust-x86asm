use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 201], OperandSize::Dword)
}

fn ucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 186695064, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 134, 152, 189, 32, 11], OperandSize::Dword)
}

fn ucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 206], OperandSize::Qword)
}

fn ucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1696415582, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 188, 74, 94, 63, 29, 101], OperandSize::Qword)
}

