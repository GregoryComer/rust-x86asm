use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 254], OperandSize::Dword)
}

fn vmovsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 16, 221], OperandSize::Qword)
}

fn vmovsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 134426412, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 20, 77, 44, 47, 3, 8], OperandSize::Dword)
}

fn vmovsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 44, 83], OperandSize::Qword)
}

fn vmovsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 239, 140, 16, 214], OperandSize::Dword)
}

fn vmovsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 151, 138, 16, 195], OperandSize::Qword)
}

fn vmovsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1144064436, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 16, 132, 192, 180, 9, 49, 68], OperandSize::Dword)
}

fn vmovsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 592434250, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 255, 141, 16, 52, 85, 74, 212, 79, 35], OperandSize::Qword)
}

fn vmovsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 203], OperandSize::Dword)
}

fn vmovsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 16, 237], OperandSize::Qword)
}

fn vmovsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 36, 72], OperandSize::Dword)
}

fn vmovsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectDisplaced(RSI, 322075892, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 158, 244, 124, 50, 19], OperandSize::Qword)
}

fn vmovsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 247, 138, 16, 211], OperandSize::Dword)
}

fn vmovsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 175, 134, 16, 226], OperandSize::Qword)
}

fn vmovsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectDisplaced(EBX, 761846341, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 139, 69, 218, 104, 45], OperandSize::Dword)
}

fn vmovsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledDisplaced(RDX, Four, 2142275454, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 255, 8, 17, 4, 149, 126, 135, 176, 127], OperandSize::Qword)
}

