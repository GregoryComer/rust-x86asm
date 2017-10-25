use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 188, 233], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 728958100, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 188, 187, 148, 4, 115, 43], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 188, 235], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RCX, 2088300963, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 188, 145, 163, 241, 120, 124], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 188, 200], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 619137823, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 188, 148, 251, 31, 75, 231, 36], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 188, 229], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 967446125, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 188, 140, 242, 109, 14, 170, 57], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 188, 195], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 508279716, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 188, 60, 117, 164, 187, 75, 30], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 154, 188, 26], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 85, 130, 188, 192], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 38102374, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 29, 138, 188, 28, 205, 102, 101, 69, 2], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1272230747, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 5, 155, 188, 52, 197, 91, 179, 212, 75], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 172, 188, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 687762335, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 188, 28, 253, 159, 107, 254, 40], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 1402223672, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 188, 161, 56, 60, 148, 83], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 61, 165, 188, 213], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RSI, 1517591039, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 21, 175, 188, 134, 255, 153, 116, 90], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 188, 44, 246], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 186, 188, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 126730, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 188, 188, 120, 10, 239, 1, 0], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 1203779595, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 217, 188, 137, 11, 56, 192, 71], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 109, 214, 188, 245], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1349077757, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 196, 188, 20, 189, 253, 74, 105, 80], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDI, 858213053, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 5, 222, 188, 167, 189, 74, 39, 51], OperandSize::Qword)
}

