use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vminpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 93, 251], OperandSize::Dword)
}

fn vminpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1269480437, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 93, 188, 187, 245, 187, 170, 75], OperandSize::Dword)
}

fn vminpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 93, 192], OperandSize::Qword)
}

fn vminpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 93, 44, 198], OperandSize::Qword)
}

fn vminpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 93, 201], OperandSize::Dword)
}

fn vminpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 991380545, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 93, 145, 65, 68, 23, 59], OperandSize::Dword)
}

fn vminpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 93, 251], OperandSize::Qword)
}

fn vminpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RCX, 428144999, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 93, 185, 103, 249, 132, 25], OperandSize::Qword)
}

fn vminpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 142, 93, 240], OperandSize::Dword)
}

fn vminpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 93, 50], OperandSize::Dword)
}

fn vminpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 157, 93, 31], OperandSize::Dword)
}

fn vminpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 213, 135, 93, 214], OperandSize::Qword)
}

fn vminpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1316395944, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 181, 143, 93, 36, 117, 168, 155, 118, 78], OperandSize::Qword)
}

fn vminpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1349066165, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 149, 153, 93, 164, 248, 181, 29, 105, 80], OperandSize::Qword)
}

fn vminpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 173, 93, 235], OperandSize::Dword)
}

fn vminpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 174, 93, 52, 79], OperandSize::Dword)
}

fn vminpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 190, 93, 3], OperandSize::Dword)
}

fn vminpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 173, 167, 93, 242], OperandSize::Qword)
}

fn vminpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RCX, 1638494562, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 181, 166, 93, 129, 98, 113, 169, 97], OperandSize::Qword)
}

fn vminpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1428574640, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 173, 183, 93, 156, 203, 176, 81, 38, 85], OperandSize::Qword)
}

fn vminpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 158, 93, 231], OperandSize::Dword)
}

fn vminpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 422943135, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 206, 93, 156, 211, 159, 153, 53, 25], OperandSize::Dword)
}

fn vminpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDI, 1789368269, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 223, 93, 183, 205, 151, 167, 106], OperandSize::Dword)
}

fn vminpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 157, 153, 93, 241], OperandSize::Qword)
}

fn vminpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1086490467, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 133, 196, 93, 28, 69, 99, 135, 194, 64], OperandSize::Qword)
}

fn vminpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 73853638, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 173, 219, 93, 52, 213, 198, 234, 102, 4], OperandSize::Qword)
}

