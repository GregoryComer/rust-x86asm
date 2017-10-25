use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 196], OperandSize::Dword)
}

fn movsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 248], OperandSize::Qword)
}

fn movsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 1893940978, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 179, 242, 62, 227, 112], OperandSize::Dword)
}

fn movsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 75105207, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 129, 183, 3, 122, 4], OperandSize::Qword)
}

fn movsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 246], OperandSize::Dword)
}

fn movsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 51], OperandSize::Dword)
}

fn movsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 16, 246], OperandSize::Qword)
}

fn movsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1566791361, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 17, 188, 81, 193, 86, 99, 93], OperandSize::Qword)
}

