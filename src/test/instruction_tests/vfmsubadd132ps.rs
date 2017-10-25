use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsubadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 151, 252], OperandSize::Dword)
}

fn vfmsubadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 151, 4, 195], OperandSize::Dword)
}

fn vfmsubadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 151, 224], OperandSize::Qword)
}

fn vfmsubadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 1335666817, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 151, 187, 129, 168, 156, 79], OperandSize::Qword)
}

fn vfmsubadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 151, 205], OperandSize::Dword)
}

fn vfmsubadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 151, 44, 119], OperandSize::Dword)
}

fn vfmsubadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 151, 217], OperandSize::Qword)
}

fn vfmsubadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 581940421, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 151, 140, 129, 197, 180, 175, 34], OperandSize::Qword)
}

fn vfmsubadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 151, 192], OperandSize::Dword)
}

fn vfmsubadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 380306797, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 151, 148, 255, 109, 5, 171, 22], OperandSize::Dword)
}

fn vfmsubadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 419198729, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 153, 151, 52, 77, 9, 119, 252, 24], OperandSize::Dword)
}

fn vfmsubadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 117, 137, 151, 231], OperandSize::Qword)
}

fn vfmsubadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 151, 52, 176], OperandSize::Qword)
}

fn vfmsubadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 53, 151, 151, 23], OperandSize::Qword)
}

fn vfmsubadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 151, 252], OperandSize::Dword)
}

fn vfmsubadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 1430873047, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 151, 168, 215, 99, 73, 85], OperandSize::Dword)
}

fn vfmsubadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 1932251646, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 186, 151, 147, 254, 209, 43, 115], OperandSize::Dword)
}

fn vfmsubadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 37, 164, 151, 230], OperandSize::Qword)
}

fn vfmsubadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 717837080, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 162, 151, 12, 213, 24, 83, 201, 42], OperandSize::Qword)
}

fn vfmsubadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 178, 151, 47], OperandSize::Qword)
}

fn vfmsubadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 250, 151, 226], OperandSize::Dword)
}

fn vfmsubadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 1818448550, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 202, 151, 175, 166, 82, 99, 108], OperandSize::Dword)
}

fn vfmsubadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 362593784, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 222, 151, 60, 149, 248, 189, 156, 21], OperandSize::Dword)
}

fn vfmsubadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 77, 178, 151, 192], OperandSize::Qword)
}

fn vfmsubadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 13, 202, 151, 19], OperandSize::Qword)
}

fn vfmsubadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 23269604, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 93, 211, 151, 20, 93, 228, 16, 99, 1], OperandSize::Qword)
}

