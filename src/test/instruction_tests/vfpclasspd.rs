use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfpclasspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 15, 102, 235, 97], OperandSize::Dword)
}

fn vfpclasspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 11, 102, 44, 123, 64], OperandSize::Dword)
}

fn vfpclasspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(IndirectDisplaced(ESI, 238993735, Some(OperandSize::Qword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 25, 102, 182, 71, 193, 62, 14, 58], OperandSize::Dword)
}

fn vfpclasspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM11)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 253, 14, 102, 227, 43], OperandSize::Qword)
}

fn vfpclasspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectDisplaced(RSI, 1966813115, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 12, 102, 190, 187, 47, 59, 117, 79], OperandSize::Qword)
}

fn vfpclasspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectDisplaced(RDX, 1274210897, Some(OperandSize::Qword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 29, 102, 146, 81, 234, 242, 75, 2], OperandSize::Qword)
}

fn vfpclasspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 47, 102, 200, 47], OperandSize::Dword)
}

fn vfpclasspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1077402033, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 44, 102, 164, 87, 177, 217, 55, 64, 113], OperandSize::Dword)
}

fn vfpclasspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 58, 102, 20, 179, 56], OperandSize::Dword)
}

fn vfpclasspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM23)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 253, 41, 102, 223, 126], OperandSize::Qword)
}

fn vfpclasspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1244449271, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 46, 102, 156, 178, 247, 201, 44, 74, 120], OperandSize::Qword)
}

fn vfpclasspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectDisplaced(RSI, 1568626711, Some(OperandSize::Qword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 62, 102, 190, 23, 88, 127, 93, 17], OperandSize::Qword)
}

fn vfpclasspd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 74, 102, 224, 30], OperandSize::Dword)
}

fn vfpclasspd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 73, 102, 20, 249, 34], OperandSize::Dword)
}

fn vfpclasspd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 89, 102, 60, 122, 98], OperandSize::Dword)
}

fn vfpclasspd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM31)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 253, 77, 102, 231, 19], OperandSize::Qword)
}

fn vfpclasspd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 391859442, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 74, 102, 156, 126, 242, 76, 91, 23, 46], OperandSize::Qword)
}

fn vfpclasspd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 94, 102, 27, 69], OperandSize::Qword)
}

