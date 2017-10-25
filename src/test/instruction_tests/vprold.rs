use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprold_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 139, 114, 205, 42], OperandSize::Dword)
}

fn vprold_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 1379477656, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 114, 136, 152, 40, 57, 82, 111], OperandSize::Dword)
}

fn vprold_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1227578556, Some(OperandSize::Dword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 153, 114, 12, 93, 188, 92, 43, 73, 13], OperandSize::Dword)
}

fn vprold_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM30)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 117, 133, 114, 206, 101], OperandSize::Qword)
}

fn vprold_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 873348857, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 130, 114, 140, 128, 249, 62, 14, 52, 108], OperandSize::Qword)
}

fn vprold_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectDisplaced(RDI, 1476590753, Some(OperandSize::Dword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 21, 157, 114, 143, 161, 252, 2, 88, 7], OperandSize::Qword)
}

fn vprold_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 114, 200, 0], OperandSize::Dword)
}

fn vprold_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 114, 8, 84], OperandSize::Dword)
}

fn vprold_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 926132964, Some(OperandSize::Dword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 189, 114, 12, 141, 228, 170, 51, 55, 68], OperandSize::Dword)
}

fn vprold_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 77, 161, 114, 200, 98], OperandSize::Qword)
}

fn vprold_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectDisplaced(RAX, 613858365, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 45, 171, 114, 136, 61, 188, 150, 36, 45], OperandSize::Qword)
}

fn vprold_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectDisplaced(RSI, 735375882, Some(OperandSize::Dword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 5, 187, 114, 142, 10, 242, 212, 43, 0], OperandSize::Qword)
}

fn vprold_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 201, 114, 205, 76], OperandSize::Dword)
}

fn vprold_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 202, 114, 9, 99], OperandSize::Dword)
}

fn vprold_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EAX, 1718888520, Some(OperandSize::Dword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 218, 114, 136, 72, 40, 116, 102, 79], OperandSize::Dword)
}

fn vprold_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 13, 207, 114, 205, 15], OperandSize::Qword)
}

fn vprold_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 207, 114, 15, 35], OperandSize::Qword)
}

fn vprold_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectDisplaced(RBX, 1246877227, Some(OperandSize::Dword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 212, 114, 139, 43, 214, 81, 74, 117], OperandSize::Qword)
}

