use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 152, 195], OperandSize::Dword)
}

fn vfmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 409057057, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 152, 139, 33, 183, 97, 24], OperandSize::Dword)
}

fn vfmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 152, 225], OperandSize::Qword)
}

fn vfmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2020968065, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 152, 4, 125, 129, 134, 117, 120], OperandSize::Qword)
}

fn vfmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 152, 255], OperandSize::Dword)
}

fn vfmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1036914093, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 152, 164, 126, 173, 13, 206, 61], OperandSize::Dword)
}

fn vfmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 152, 231], OperandSize::Qword)
}

fn vfmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 152, 44, 135], OperandSize::Qword)
}

fn vfmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 152, 222], OperandSize::Dword)
}

fn vfmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 152, 20, 73], OperandSize::Dword)
}

fn vfmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 157, 152, 39], OperandSize::Dword)
}

fn vfmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 69, 130, 152, 238], OperandSize::Qword)
}

fn vfmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 431792385, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 134, 152, 132, 131, 1, 161, 188, 25], OperandSize::Qword)
}

fn vfmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RBX, 1246525465, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 101, 147, 152, 139, 25, 120, 76, 74], OperandSize::Qword)
}

fn vfmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 152, 247], OperandSize::Dword)
}

fn vfmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 1928948034, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 172, 152, 144, 66, 105, 249, 114], OperandSize::Dword)
}

fn vfmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 188, 152, 31], OperandSize::Dword)
}

fn vfmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 93, 174, 152, 251], OperandSize::Qword)
}

fn vfmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 45, 175, 152, 38], OperandSize::Qword)
}

fn vfmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 548709296, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 186, 152, 156, 114, 176, 163, 180, 32], OperandSize::Qword)
}

fn vfmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 219, 152, 214], OperandSize::Dword)
}

fn vfmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 206, 152, 30], OperandSize::Dword)
}

fn vfmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1965328732, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 221, 152, 140, 191, 92, 137, 36, 117], OperandSize::Dword)
}

fn vfmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 93, 243, 152, 193], OperandSize::Qword)
}

fn vfmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1041707121, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 152, 20, 181, 113, 48, 23, 62], OperandSize::Qword)
}

fn vfmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1085769646, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 61, 214, 152, 140, 200, 174, 135, 183, 64], OperandSize::Qword)
}

