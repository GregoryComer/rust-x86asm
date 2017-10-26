use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 111, 235], OperandSize::Dword)
}

#[test]
fn vmovdqa32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 111, 44, 70], OperandSize::Dword)
}

#[test]
fn vmovdqa32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 125, 142, 111, 239], OperandSize::Qword)
}

#[test]
fn vmovdqa32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 111, 28, 246], OperandSize::Qword)
}

#[test]
fn vmovdqa32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 111, 250], OperandSize::Dword)
}

#[test]
fn vmovdqa32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 111, 23], OperandSize::Dword)
}

#[test]
fn vmovdqa32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 125, 171, 111, 223], OperandSize::Qword)
}

#[test]
fn vmovdqa32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 175, 111, 20, 72], OperandSize::Qword)
}

#[test]
fn vmovdqa32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 111, 254], OperandSize::Dword)
}

#[test]
fn vmovdqa32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 2009496064, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 111, 20, 77, 0, 122, 198, 119], OperandSize::Dword)
}

#[test]
fn vmovdqa32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 125, 203, 111, 216], OperandSize::Qword)
}

#[test]
fn vmovdqa32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 205, 111, 36, 82], OperandSize::Qword)
}

#[test]
fn vmovdqa32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 111, 250], OperandSize::Dword)
}

#[test]
fn vmovdqa32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1403951523, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 148, 248, 163, 153, 174, 83], OperandSize::Dword)
}

#[test]
fn vmovdqa32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 125, 141, 111, 225], OperandSize::Qword)
}

#[test]
fn vmovdqa32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 861076803, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 140, 121, 67, 253, 82, 51], OperandSize::Qword)
}

#[test]
fn vmovdqa32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 111, 244], OperandSize::Dword)
}

#[test]
fn vmovdqa32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 127, 1], OperandSize::Dword)
}

#[test]
fn vmovdqa32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 125, 171, 111, 240], OperandSize::Qword)
}

#[test]
fn vmovdqa32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectDisplaced(RAX, 571372730, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 40, 127, 128, 186, 116, 14, 34], OperandSize::Qword)
}

#[test]
fn vmovdqa32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 111, 221], OperandSize::Dword)
}

#[test]
fn vmovdqa32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectDisplaced(EAX, 1781254607, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 160, 207, 201, 43, 106], OperandSize::Dword)
}

#[test]
fn vmovdqa32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 125, 207, 111, 219], OperandSize::Qword)
}

#[test]
fn vmovdqa32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledDisplaced(RCX, Four, 906195778, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 125, 72, 127, 20, 141, 66, 115, 3, 54], OperandSize::Qword)
}

