use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 205], OperandSize::Dword)
}

fn pmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 977278461, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 167, 253, 21, 64, 58], OperandSize::Dword)
}

fn pmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 246], OperandSize::Qword)
}

fn pmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1709813562, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 172, 95, 58, 175, 233, 101], OperandSize::Qword)
}

