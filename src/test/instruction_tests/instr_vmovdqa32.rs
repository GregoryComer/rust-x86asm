use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 111, 236], OperandSize::Dword)
}

#[test]
fn vmovdqa32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 111, 52, 183], OperandSize::Dword)
}

#[test]
fn vmovdqa32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 125, 137, 111, 215], OperandSize::Qword)
}

#[test]
fn vmovdqa32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 111, 4, 217], OperandSize::Qword)
}

#[test]
fn vmovdqa32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 111, 232], OperandSize::Dword)
}

#[test]
fn vmovdqa32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 327514760, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 111, 12, 213, 136, 122, 133, 19], OperandSize::Dword)
}

#[test]
fn vmovdqa32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 125, 169, 111, 238], OperandSize::Qword)
}

#[test]
fn vmovdqa32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 125, 171, 111, 36, 129], OperandSize::Qword)
}

#[test]
fn vmovdqa32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 111, 194], OperandSize::Dword)
}

#[test]
fn vmovdqa32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EAX, 971305154, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 111, 184, 194, 240, 228, 57], OperandSize::Dword)
}

#[test]
fn vmovdqa32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 125, 207, 111, 201], OperandSize::Qword)
}

#[test]
fn vmovdqa32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM12)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 204, 111, 38], OperandSize::Qword)
}

#[test]
fn vmovdqa32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 111, 204], OperandSize::Dword)
}

#[test]
fn vmovdqa32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1945330797, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 164, 200, 109, 100, 243, 115], OperandSize::Dword)
}

#[test]
fn vmovdqa32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 125, 141, 111, 244], OperandSize::Qword)
}

#[test]
fn vmovdqa32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 127, 32], OperandSize::Qword)
}

#[test]
fn vmovdqa32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 111, 198], OperandSize::Dword)
}

#[test]
fn vmovdqa32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectDisplaced(EDI, 907338357, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 127, 159, 117, 226, 20, 54], OperandSize::Dword)
}

#[test]
fn vmovdqa32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 111, 225], OperandSize::Qword)
}

#[test]
fn vmovdqa32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 40, 127, 56], OperandSize::Qword)
}

#[test]
fn vmovdqa32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 111, 254], OperandSize::Dword)
}

#[test]
fn vmovdqa32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 4, 193], OperandSize::Dword)
}

#[test]
fn vmovdqa32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 125, 203, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqa32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 772304330, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 72, 127, 52, 205, 202, 109, 8, 46], OperandSize::Qword)
}

