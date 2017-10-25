use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 244400091, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 22, 144, 219, 63, 145, 14], OperandSize::Dword)
}

fn vmovhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 697559440, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 22, 60, 181, 144, 233, 147, 41], OperandSize::Qword)
}

fn vmovhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 22, 44, 146], OperandSize::Dword)
}

fn vmovhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 1442460191, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 22, 168, 31, 50, 250, 85], OperandSize::Qword)
}

fn vmovhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 414969637, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 172, 159, 37, 239, 187, 24], OperandSize::Dword)
}

fn vmovhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectDisplaced(RDX, 88235215, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 130, 207, 92, 66, 5], OperandSize::Qword)
}

fn vmovhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 861144349, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 148, 178, 29, 5, 84, 51], OperandSize::Dword)
}

fn vmovhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 2016319460, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 156, 91, 228, 151, 46, 120], OperandSize::Qword)
}

