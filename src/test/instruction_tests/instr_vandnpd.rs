use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 85, 241], OperandSize::Dword)
}

#[test]
fn vandnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 2052971500, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 85, 144, 236, 219, 93, 122], OperandSize::Dword)
}

#[test]
fn vandnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 85, 213], OperandSize::Qword)
}

#[test]
fn vandnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 844164427, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 85, 132, 67, 75, 237, 80, 50], OperandSize::Qword)
}

#[test]
fn vandnpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 85, 244], OperandSize::Dword)
}

#[test]
fn vandnpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 704883341, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 85, 60, 205, 141, 170, 3, 42], OperandSize::Dword)
}

#[test]
fn vandnpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 85, 240], OperandSize::Qword)
}

#[test]
fn vandnpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 19419655, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 85, 162, 7, 82, 40, 1], OperandSize::Qword)
}

#[test]
fn vandnpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 138, 85, 241], OperandSize::Dword)
}

#[test]
fn vandnpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 2016249635, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 142, 85, 150, 35, 135, 45, 120], OperandSize::Dword)
}

#[test]
fn vandnpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1915276522, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 159, 85, 156, 75, 234, 204, 40, 114], OperandSize::Dword)
}

#[test]
fn vandnpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 181, 135, 85, 225], OperandSize::Qword)
}

#[test]
fn vandnpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 2038908256, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 157, 130, 85, 132, 209, 96, 69, 135, 121], OperandSize::Qword)
}

#[test]
fn vandnpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RAX, 679496477, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 213, 159, 85, 160, 29, 75, 128, 40], OperandSize::Qword)
}

#[test]
fn vandnpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 174, 85, 203], OperandSize::Dword)
}

#[test]
fn vandnpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 169, 85, 12, 150], OperandSize::Dword)
}

#[test]
fn vandnpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 191995727, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 186, 85, 137, 79, 159, 113, 11], OperandSize::Dword)
}

#[test]
fn vandnpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 173, 85, 205], OperandSize::Qword)
}

#[test]
fn vandnpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 355802486, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 174, 85, 132, 193, 118, 29, 53, 21], OperandSize::Qword)
}

#[test]
fn vandnpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RBX, 1814868155, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 133, 185, 85, 187, 187, 176, 44, 108], OperandSize::Qword)
}

#[test]
fn vandnpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 201, 85, 197], OperandSize::Dword)
}

#[test]
fn vandnpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 85, 36, 83], OperandSize::Dword)
}

#[test]
fn vandnpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 221, 85, 36, 158], OperandSize::Dword)
}

#[test]
fn vandnpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 189, 203, 85, 234], OperandSize::Qword)
}

#[test]
fn vandnpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 157, 202, 85, 39], OperandSize::Qword)
}

#[test]
fn vandnpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDX, 1702024866, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 213, 212, 85, 170, 162, 214, 114, 101], OperandSize::Qword)
}

