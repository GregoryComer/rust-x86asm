use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn packusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 244], OperandSize::Dword)
}

fn packusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 110233717, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 52, 181, 117, 8, 146, 6], OperandSize::Dword)
}

fn packusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 201], OperandSize::Qword)
}

fn packusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDX, 1712030559, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 178, 95, 131, 11, 102], OperandSize::Qword)
}

