use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 16, 247], OperandSize::Dword)
}

fn vmovss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 16, 249], OperandSize::Qword)
}

fn vmovss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 133712810, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 132, 206, 170, 75, 248, 7], OperandSize::Dword)
}

fn vmovss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 28, 126], OperandSize::Qword)
}

fn vmovss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 86, 138, 16, 210], OperandSize::Dword)
}

fn vmovss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 110, 141, 16, 221], OperandSize::Qword)
}

fn vmovss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 181596108, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 16, 138, 204, 239, 210, 10], OperandSize::Dword)
}

fn vmovss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 137, 16, 4, 194], OperandSize::Qword)
}

fn vmovss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 16, 242], OperandSize::Dword)
}

fn vmovss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 221], OperandSize::Qword)
}

fn vmovss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(ECX, 1591773270, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 177, 86, 136, 224, 94], OperandSize::Dword)
}

fn vmovss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 44, 255], OperandSize::Qword)
}

fn vmovss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 118, 141, 16, 203], OperandSize::Dword)
}

fn vmovss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 118, 139, 16, 206], OperandSize::Qword)
}

fn vmovss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 0], OperandSize::Dword)
}

fn vmovss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(RBX, 1050105709, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 8, 17, 147, 109, 87, 151, 62], OperandSize::Qword)
}

