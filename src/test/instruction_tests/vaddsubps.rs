use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaddsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 208, 252], OperandSize::Dword)
}

fn vaddsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 243646979, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 208, 140, 215, 3, 194, 133, 14], OperandSize::Dword)
}

fn vaddsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 208, 242], OperandSize::Qword)
}

fn vaddsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 809074430, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 208, 4, 85, 254, 126, 57, 48], OperandSize::Qword)
}

fn vaddsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 208, 237], OperandSize::Dword)
}

fn vaddsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1430148994, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 208, 20, 77, 130, 87, 62, 85], OperandSize::Dword)
}

fn vaddsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 208, 203], OperandSize::Qword)
}

fn vaddsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RBX, 1968103637, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 208, 179, 213, 224, 78, 117], OperandSize::Qword)
}

