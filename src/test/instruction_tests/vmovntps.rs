use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 19], OperandSize::Dword)
}

fn vmovntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(RBX, 106148086, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 147, 246, 176, 83, 6], OperandSize::Qword)
}

fn vmovntps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1289687003, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 60, 197, 219, 15, 223, 76], OperandSize::Dword)
}

fn vmovntps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 327143038, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 44, 205, 126, 206, 127, 19], OperandSize::Qword)
}

fn vmovntps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1721610740, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 60, 69, 244, 177, 157, 102], OperandSize::Dword)
}

fn vmovntps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 43, 24], OperandSize::Qword)
}

fn vmovntps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EBX, Two, 196329150, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 28, 93, 190, 190, 179, 11], OperandSize::Dword)
}

fn vmovntps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(RSI, 1468313418, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 40, 43, 134, 74, 175, 132, 87], OperandSize::Qword)
}

fn vmovntps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(ESI, 337821133, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 142, 205, 189, 34, 20], OperandSize::Dword)
}

fn vmovntps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 72, 43, 35], OperandSize::Qword)
}

