use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 89, 201], OperandSize::Dword)
}

#[test]
fn vmulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1314773240, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 89, 28, 245, 248, 216, 93, 78], OperandSize::Dword)
}

#[test]
fn vmulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 89, 220], OperandSize::Qword)
}

#[test]
fn vmulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 772885604, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 89, 148, 209, 100, 76, 17, 46], OperandSize::Qword)
}

#[test]
fn vmulpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 89, 222], OperandSize::Dword)
}

#[test]
fn vmulpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 89, 28, 185], OperandSize::Dword)
}

#[test]
fn vmulpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 89, 240], OperandSize::Qword)
}

#[test]
fn vmulpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 613859182, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 89, 188, 73, 110, 191, 150, 36], OperandSize::Qword)
}

#[test]
fn vmulpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 89, 195], OperandSize::Dword)
}

#[test]
fn vmulpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 141, 89, 42], OperandSize::Dword)
}

#[test]
fn vmulpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 276631245, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 154, 89, 12, 205, 205, 14, 125, 16], OperandSize::Dword)
}

#[test]
fn vmulpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 221, 134, 89, 248], OperandSize::Qword)
}

#[test]
fn vmulpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 237, 132, 89, 4, 146], OperandSize::Qword)
}

#[test]
fn vmulpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 229, 159, 89, 16], OperandSize::Qword)
}

#[test]
fn vmulpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 173, 89, 206], OperandSize::Dword)
}

#[test]
fn vmulpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 89, 52, 122], OperandSize::Dword)
}

#[test]
fn vmulpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 396325377, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 185, 89, 12, 157, 1, 114, 159, 23], OperandSize::Dword)
}

#[test]
fn vmulpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 149, 166, 89, 199], OperandSize::Qword)
}

#[test]
fn vmulpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RDI, 1009688929, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 189, 161, 89, 135, 97, 161, 46, 60], OperandSize::Qword)
}

#[test]
fn vmulpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 189, 89, 52, 216], OperandSize::Qword)
}

#[test]
fn vmulpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 188, 89, 221], OperandSize::Dword)
}

#[test]
fn vmulpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDI, 577005292, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 207, 89, 159, 236, 102, 100, 34], OperandSize::Dword)
}

#[test]
fn vmulpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 223, 89, 28, 118], OperandSize::Dword)
}

#[test]
fn vmulpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 205, 146, 89, 205], OperandSize::Qword)
}

#[test]
fn vmulpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 181, 207, 89, 60, 193], OperandSize::Qword)
}

#[test]
fn vmulpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 215, 89, 7], OperandSize::Qword)
}

