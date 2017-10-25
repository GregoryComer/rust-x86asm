use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 158, 195], OperandSize::Dword)
}

fn vfnmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1235232411, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 158, 148, 71, 155, 38, 160, 73], OperandSize::Dword)
}

fn vfnmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 158, 202], OperandSize::Qword)
}

fn vfnmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 158, 7], OperandSize::Qword)
}

fn vfnmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 158, 221], OperandSize::Dword)
}

fn vfnmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 753730996, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 158, 170, 180, 5, 237, 44], OperandSize::Dword)
}

fn vfnmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 158, 216], OperandSize::Qword)
}

fn vfnmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 1306519653, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 158, 134, 101, 232, 223, 77], OperandSize::Qword)
}

fn vfnmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 158, 214], OperandSize::Dword)
}

fn vfnmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 158, 52, 65], OperandSize::Dword)
}

fn vfnmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1315660189, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 153, 158, 190, 157, 97, 107, 78], OperandSize::Dword)
}

fn vfnmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 85, 140, 158, 225], OperandSize::Qword)
}

fn vfnmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 158, 4, 78], OperandSize::Qword)
}

fn vfnmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 151, 158, 11], OperandSize::Qword)
}

fn vfnmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 158, 207], OperandSize::Dword)
}

fn vfnmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1972903221, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 158, 172, 86, 53, 29, 152, 117], OperandSize::Dword)
}

fn vfnmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 187, 158, 52, 139], OperandSize::Dword)
}

fn vfnmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 109, 169, 158, 249], OperandSize::Qword)
}

fn vfnmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 117, 169, 158, 20, 130], OperandSize::Qword)
}

fn vfnmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 77, 186, 158, 4, 176], OperandSize::Qword)
}

fn vfnmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 159, 158, 218], OperandSize::Dword)
}

fn vfnmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 201, 158, 20, 254], OperandSize::Dword)
}

fn vfnmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 366652682, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 158, 188, 182, 10, 173, 218, 21], OperandSize::Dword)
}

fn vfnmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 37, 214, 158, 201], OperandSize::Qword)
}

fn vfnmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RAX, 192859961, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 158, 160, 57, 207, 126, 11], OperandSize::Qword)
}

fn vfnmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RDI, 212576405, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 69, 223, 158, 191, 149, 168, 171, 12], OperandSize::Qword)
}

