use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 973534923, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 148, 223, 203, 246, 6, 58], OperandSize::Dword)
}

fn movlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 16], OperandSize::Qword)
}

fn movlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 1], OperandSize::Dword)
}

fn movlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1565457772, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 12, 125, 108, 253, 78, 93], OperandSize::Qword)
}

