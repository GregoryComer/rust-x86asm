use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 183, 237], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1977205209, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 183, 180, 192, 217, 193, 217, 117], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 183, 241], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 183, 12, 199], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 244], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1925684626, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 183, 28, 189, 146, 157, 199, 114], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 206], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1911753062, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 12, 149, 102, 9, 243, 113], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 183, 218], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1088548123, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 183, 52, 213, 27, 237, 225, 64], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 158, 183, 20, 95], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 77, 135, 183, 194], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RAX, 771687221, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 61, 142, 183, 176, 53, 3, 255, 45], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 800553910, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 93, 148, 183, 148, 185, 182, 123, 183, 47], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 183, 254], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 183, 44, 182], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 185, 183, 10], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 21, 174, 183, 195], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 53, 165, 183, 34], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 384606911, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 191, 183, 4, 221, 191, 162, 236, 22], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 251, 183, 199], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 1379588153, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 183, 169, 57, 216, 58, 82], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 223, 183, 60, 214], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 125, 178, 183, 243], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RSI, 2008376570, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 196, 183, 182, 250, 100, 181, 119], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 798859945, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 29, 214, 183, 164, 112, 169, 162, 157, 47], OperandSize::Qword)
}

