use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn addsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 214], OperandSize::Dword)
}

fn addsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1226758138, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 164, 83, 250, 215, 30, 73], OperandSize::Dword)
}

fn addsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 245], OperandSize::Qword)
}

fn addsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDX, 1148050466, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 130, 34, 220, 109, 68], OperandSize::Qword)
}

