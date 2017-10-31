use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 184, 202], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1993760737, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 184, 176, 225, 95, 214, 118], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 184, 219], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 184, 36, 154], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 184, 233], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 184, 26], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 184, 228], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1135314118, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 184, 164, 79, 198, 132, 171, 67], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 184, 200], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 184, 35], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 195540455, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 159, 184, 156, 112, 231, 181, 167, 11], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 117, 139, 184, 236], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 533835723, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 184, 164, 88, 203, 175, 209, 31], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RBX, 1695654437, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 154, 184, 131, 37, 162, 17, 101], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 184, 205], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 184, 52, 207], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1499654862, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 191, 184, 52, 253, 206, 234, 98, 89], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 101, 161, 184, 248], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectDisplaced(RDX, 991274464, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 85, 165, 184, 186, 224, 165, 21, 59], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1920237657, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 180, 184, 20, 141, 89, 128, 116, 114], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 156, 184, 203], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 230856630, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 184, 180, 248, 182, 151, 194, 13], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 277177042, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 219, 184, 164, 138, 210, 98, 133, 16], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 61, 148, 184, 224], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 85, 194, 184, 40], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDX, 1101333107, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 223, 184, 130, 115, 2, 165, 65], OperandSize::Qword)
}

