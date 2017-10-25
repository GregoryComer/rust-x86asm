use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn comiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 234], OperandSize::Dword)
}

fn comiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1954074087, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 28, 85, 231, 205, 120, 116], OperandSize::Dword)
}

fn comiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 195], OperandSize::Qword)
}

fn comiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RSI, 2034687967, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 47, 150, 223, 223, 70, 121], OperandSize::Qword)
}

