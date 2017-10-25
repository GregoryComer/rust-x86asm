use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 111, 236], OperandSize::Dword)
}

#[test]
fn vmovdqa64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 111, 6], OperandSize::Dword)
}

#[test]
fn vmovdqa64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 253, 141, 111, 208], OperandSize::Qword)
}

#[test]
fn vmovdqa64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RDX, 1830166324, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 137, 111, 138, 52, 31, 22, 109], OperandSize::Qword)
}

#[test]
fn vmovdqa64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 111, 247], OperandSize::Dword)
}

#[test]
fn vmovdqa64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 111, 19], OperandSize::Dword)
}

#[test]
fn vmovdqa64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 253, 169, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqa64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1268457126, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 171, 111, 180, 120, 166, 30, 155, 75], OperandSize::Qword)
}

#[test]
fn vmovdqa64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 111, 235], OperandSize::Dword)
}

#[test]
fn vmovdqa64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1186389387, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 111, 172, 182, 139, 221, 182, 70], OperandSize::Dword)
}

#[test]
fn vmovdqa64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 253, 207, 111, 243], OperandSize::Qword)
}

#[test]
fn vmovdqa64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectDisplaced(RDX, 1821422353, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 202, 111, 178, 17, 179, 144, 108], OperandSize::Qword)
}

#[test]
fn vmovdqa64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 111, 245], OperandSize::Dword)
}

#[test]
fn vmovdqa64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1279407960, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 8, 127, 172, 64, 88, 55, 66, 76], OperandSize::Dword)
}

#[test]
fn vmovdqa64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 142, 111, 194], OperandSize::Qword)
}

#[test]
fn vmovdqa64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 127, 2], OperandSize::Qword)
}

#[test]
fn vmovdqa64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 111, 249], OperandSize::Dword)
}

#[test]
fn vmovdqa64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(EDI, 657592330, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 40, 127, 167, 10, 16, 50, 39], OperandSize::Dword)
}

#[test]
fn vmovdqa64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 253, 175, 111, 207], OperandSize::Qword)
}

#[test]
fn vmovdqa64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 127, 20, 137], OperandSize::Qword)
}

#[test]
fn vmovdqa64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 111, 192], OperandSize::Dword)
}

#[test]
fn vmovdqa64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1562415339, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 164, 254, 235, 144, 32, 93], OperandSize::Dword)
}

#[test]
fn vmovdqa64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 203, 111, 234], OperandSize::Qword)
}

#[test]
fn vmovdqa64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(RSI, 1143895016, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 182, 232, 115, 46, 68], OperandSize::Qword)
}

