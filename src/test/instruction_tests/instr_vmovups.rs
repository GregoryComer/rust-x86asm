use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 224], OperandSize::Dword)
}

#[test]
fn vmovups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 38], OperandSize::Dword)
}

#[test]
fn vmovups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 194], OperandSize::Qword)
}

#[test]
fn vmovups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1894087047, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 185, 135, 121, 229, 112], OperandSize::Qword)
}

#[test]
fn vmovups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 244], OperandSize::Dword)
}

#[test]
fn vmovups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDX, 1195537101, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 178, 205, 114, 66, 71], OperandSize::Dword)
}

#[test]
fn vmovups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 238], OperandSize::Qword)
}

#[test]
fn vmovups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1874875484, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 44, 93, 92, 84, 192, 111], OperandSize::Qword)
}

#[test]
fn vmovups_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 16, 238], OperandSize::Dword)
}

#[test]
fn vmovups_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 16, 36, 255], OperandSize::Dword)
}

#[test]
fn vmovups_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 124, 140, 16, 203], OperandSize::Qword)
}

#[test]
fn vmovups_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 408781809, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 137, 16, 60, 205, 241, 131, 93, 24], OperandSize::Qword)
}

#[test]
fn vmovups_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 16, 214], OperandSize::Dword)
}

#[test]
fn vmovups_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 16, 17], OperandSize::Dword)
}

#[test]
fn vmovups_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 173, 16, 224], OperandSize::Qword)
}

#[test]
fn vmovups_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1740158854, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 174, 16, 4, 149, 134, 183, 184, 103], OperandSize::Qword)
}

#[test]
fn vmovups_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 16, 222], OperandSize::Dword)
}

#[test]
fn vmovups_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1209746169, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 204, 16, 180, 214, 249, 66, 27, 72], OperandSize::Dword)
}

#[test]
fn vmovups_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 124, 203, 16, 200], OperandSize::Qword)
}

#[test]
fn vmovups_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 16, 42], OperandSize::Qword)
}

#[test]
fn vmovups_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 206], OperandSize::Dword)
}

#[test]
fn vmovups_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1288913289, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 164, 255, 137, 65, 211, 76], OperandSize::Dword)
}

#[test]
fn vmovups_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 247], OperandSize::Qword)
}

#[test]
fn vmovups_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 57], OperandSize::Qword)
}

#[test]
fn vmovups_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 240], OperandSize::Dword)
}

#[test]
fn vmovups_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1471772158, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 28, 69, 254, 117, 185, 87], OperandSize::Dword)
}

#[test]
fn vmovups_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 201], OperandSize::Qword)
}

#[test]
fn vmovups_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RSI, 432202336, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 142, 96, 226, 194, 25], OperandSize::Qword)
}

#[test]
fn vmovups_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 16, 195], OperandSize::Dword)
}

#[test]
fn vmovups_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1655271749, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 172, 193, 69, 113, 169, 98], OperandSize::Dword)
}

#[test]
fn vmovups_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 124, 139, 16, 234], OperandSize::Qword)
}

#[test]
fn vmovups_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 17, 28, 90], OperandSize::Qword)
}

#[test]
fn vmovups_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 16, 233], OperandSize::Dword)
}

#[test]
fn vmovups_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1769599968, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 164, 241, 224, 243, 121, 105], OperandSize::Dword)
}

#[test]
fn vmovups_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 124, 174, 16, 196], OperandSize::Qword)
}

#[test]
fn vmovups_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RBX, 649575218, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 187, 50, 187, 183, 38], OperandSize::Qword)
}

#[test]
fn vmovups_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 16, 240], OperandSize::Dword)
}

#[test]
fn vmovups_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(ESI, 277502857, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 182, 137, 91, 138, 16], OperandSize::Dword)
}

#[test]
fn vmovups_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 124, 202, 16, 241], OperandSize::Qword)
}

#[test]
fn vmovups_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RAX, 1249629800, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 124, 72, 17, 168, 104, 214, 123, 74], OperandSize::Qword)
}

