use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermt2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 127, 206], OperandSize::Dword)
}

fn vpermt2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 618442345, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 127, 188, 215, 105, 174, 220, 36], OperandSize::Dword)
}

fn vpermt2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1816287258, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 154, 127, 164, 122, 26, 88, 66, 108], OperandSize::Dword)
}

fn vpermt2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 229, 129, 127, 214], OperandSize::Qword)
}

fn vpermt2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2045953273, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 157, 133, 127, 20, 221, 249, 196, 242, 121], OperandSize::Qword)
}

fn vpermt2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 147, 127, 46], OperandSize::Qword)
}

fn vpermt2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 127, 231], OperandSize::Dword)
}

fn vpermt2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 127, 20, 89], OperandSize::Dword)
}

fn vpermt2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 188, 127, 52, 150], OperandSize::Dword)
}

fn vpermt2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 173, 169, 127, 233], OperandSize::Qword)
}

fn vpermt2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RAX, 430122420, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 237, 162, 127, 184, 180, 37, 163, 25], OperandSize::Qword)
}

fn vpermt2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1047746824, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 245, 180, 127, 188, 219, 8, 89, 115, 62], OperandSize::Qword)
}

fn vpermt2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 203, 127, 195], OperandSize::Dword)
}

fn vpermt2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 127, 11], OperandSize::Dword)
}

fn vpermt2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 220, 127, 55], OperandSize::Dword)
}

fn vpermt2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 229, 198, 127, 240], OperandSize::Qword)
}

fn vpermt2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1400394640, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 221, 201, 127, 164, 70, 144, 83, 120, 83], OperandSize::Qword)
}

fn vpermt2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1854731135, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 237, 212, 127, 180, 192, 127, 243, 140, 110], OperandSize::Qword)
}

