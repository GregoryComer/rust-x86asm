use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 64, 253], OperandSize::Dword)
}

fn vpmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1589255192, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 64, 188, 211, 24, 28, 186, 94], OperandSize::Dword)
}

fn vpmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 64, 203], OperandSize::Qword)
}

fn vpmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 64, 4, 206], OperandSize::Qword)
}

fn vpmulld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 64, 209], OperandSize::Dword)
}

fn vpmulld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 64, 4, 112], OperandSize::Dword)
}

fn vpmulld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 64, 227], OperandSize::Qword)
}

fn vpmulld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 761948483, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 64, 140, 186, 67, 105, 106, 45], OperandSize::Qword)
}

fn vpmulld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 64, 215], OperandSize::Dword)
}

fn vpmulld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 1752404480, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 64, 155, 0, 146, 115, 104], OperandSize::Dword)
}

fn vpmulld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 64, 52, 247], OperandSize::Dword)
}

fn vpmulld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 45, 143, 64, 221], OperandSize::Qword)
}

fn vpmulld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 53, 137, 64, 4, 201], OperandSize::Qword)
}

fn vpmulld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 53, 156, 64, 51], OperandSize::Qword)
}

fn vpmulld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 64, 218], OperandSize::Dword)
}

fn vpmulld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 64, 52, 200], OperandSize::Dword)
}

fn vpmulld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1361786955, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 187, 64, 154, 75, 56, 43, 81], OperandSize::Dword)
}

fn vpmulld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 21, 172, 64, 231], OperandSize::Qword)
}

fn vpmulld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1352842884, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 64, 188, 80, 132, 190, 162, 80], OperandSize::Qword)
}

fn vpmulld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 101, 185, 64, 52, 126], OperandSize::Qword)
}

fn vpmulld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 202, 64, 223], OperandSize::Dword)
}

fn vpmulld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1367548862, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 64, 132, 192, 190, 35, 131, 81], OperandSize::Dword)
}

fn vpmulld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 68591460, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 221, 64, 148, 202, 100, 159, 22, 4], OperandSize::Dword)
}

fn vpmulld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 61, 193, 64, 221], OperandSize::Qword)
}

fn vpmulld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RCX, 404580090, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 109, 198, 64, 145, 250, 102, 29, 24], OperandSize::Qword)
}

fn vpmulld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1979227709, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 37, 219, 64, 188, 158, 61, 158, 248, 117], OperandSize::Qword)
}

