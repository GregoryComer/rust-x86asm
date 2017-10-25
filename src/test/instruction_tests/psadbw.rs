use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 221], OperandSize::Dword)
}

fn psadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1274932482, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 188, 80, 2, 237, 253, 75], OperandSize::Dword)
}

fn psadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 208], OperandSize::Qword)
}

fn psadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1661473882, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 52, 77, 90, 20, 8, 99], OperandSize::Qword)
}

fn psadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 251], OperandSize::Dword)
}

fn psadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 903001775, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 148, 95, 175, 182, 210, 53], OperandSize::Dword)
}

fn psadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 205], OperandSize::Qword)
}

fn psadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDX, 1563185004, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 162, 108, 79, 44, 93], OperandSize::Qword)
}

