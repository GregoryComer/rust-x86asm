use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 184, 248], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1226825400, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 184, 140, 130, 184, 222, 31, 73], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 184, 234], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 184, 12, 130], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 184, 251], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1765071720, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 184, 4, 253, 104, 219, 52, 105], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 184, 247], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 183655102, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 184, 148, 182, 190, 90, 242, 10], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 184, 207], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 184, 30], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 184, 4, 118], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 53, 143, 184, 237], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 21, 137, 184, 44, 179], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 314720314, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 125, 157, 184, 156, 119, 58, 64, 194, 18], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 184, 242], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 184, 3], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 1559064363, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 188, 184, 187, 43, 111, 237, 92], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 85, 161, 184, 231], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1388594306, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 171, 184, 140, 79, 130, 68, 196, 82], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RCX, 1564983885, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 182, 184, 185, 77, 194, 71, 93], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 219, 184, 226], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 2033703540, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 184, 172, 70, 116, 218, 55, 121], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 223, 184, 40], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 53, 153, 184, 205], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 29, 195, 184, 54], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 37, 210, 184, 51], OperandSize::Qword)
}

