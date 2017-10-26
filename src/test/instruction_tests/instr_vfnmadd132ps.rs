use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 156, 221], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 67726318, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 156, 12, 69, 238, 107, 9, 4], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 156, 197], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 632008383, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 156, 188, 243, 191, 174, 171, 37], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 156, 239], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 156, 44, 74], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 156, 218], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 156, 20, 242], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 156, 226], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1295506133, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 156, 4, 117, 213, 218, 55, 77], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 158, 156, 48], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 21, 141, 156, 223], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 29, 139, 156, 63], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 133345701, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 148, 156, 164, 128, 165, 177, 242, 7], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 156, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 476808014, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 170, 156, 132, 136, 78, 131, 107, 28], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1319147037, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 185, 156, 148, 82, 29, 150, 160, 78], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 13, 175, 156, 208], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 259007493, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 29, 163, 156, 188, 216, 5, 36, 112, 15], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 45, 185, 156, 56], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 217, 156, 204], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 785396046, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 156, 36, 77, 78, 49, 208, 46], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1475341514, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 222, 156, 172, 222, 202, 236, 239, 87], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 101, 223, 156, 206], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 555572378, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 93, 195, 156, 172, 249, 154, 92, 29, 33], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 464602325, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 156, 12, 205, 213, 68, 177, 27], OperandSize::Qword)
}

