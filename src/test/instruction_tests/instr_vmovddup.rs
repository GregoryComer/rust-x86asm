use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 223], OperandSize::Dword)
}

#[test]
fn vmovddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 34], OperandSize::Dword)
}

#[test]
fn vmovddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 208], OperandSize::Qword)
}

#[test]
fn vmovddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1651643508, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 140, 138, 116, 20, 114, 98], OperandSize::Qword)
}

#[test]
fn vmovddup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 232], OperandSize::Dword)
}

#[test]
fn vmovddup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 886040204, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 148, 142, 140, 230, 207, 52], OperandSize::Dword)
}

#[test]
fn vmovddup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 232], OperandSize::Qword)
}

#[test]
fn vmovddup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1729245372, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 28, 245, 188, 48, 18, 103], OperandSize::Qword)
}

#[test]
fn vmovddup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 18, 249], OperandSize::Dword)
}

#[test]
fn vmovddup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 713369430, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 18, 52, 205, 86, 39, 133, 42], OperandSize::Dword)
}

#[test]
fn vmovddup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 255, 137, 18, 227], OperandSize::Qword)
}

#[test]
fn vmovddup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 18, 7], OperandSize::Qword)
}

#[test]
fn vmovddup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 18, 223], OperandSize::Dword)
}

#[test]
fn vmovddup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1022467070, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 18, 28, 205, 254, 155, 241, 60], OperandSize::Dword)
}

#[test]
fn vmovddup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 255, 169, 18, 241], OperandSize::Qword)
}

#[test]
fn vmovddup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 93679826, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 172, 18, 180, 83, 210, 112, 149, 5], OperandSize::Qword)
}

#[test]
fn vmovddup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 206, 18, 221], OperandSize::Dword)
}

#[test]
fn vmovddup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ECX, 1448487275, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 18, 169, 107, 41, 86, 86], OperandSize::Dword)
}

#[test]
fn vmovddup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 255, 201, 18, 200], OperandSize::Qword)
}

#[test]
fn vmovddup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 2056410567, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 204, 18, 132, 121, 199, 85, 146, 122], OperandSize::Qword)
}

