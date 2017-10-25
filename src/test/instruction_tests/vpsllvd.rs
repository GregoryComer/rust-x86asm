use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsllvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 71, 241], OperandSize::Dword)
}

fn vpsllvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 71, 39], OperandSize::Dword)
}

fn vpsllvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 71, 228], OperandSize::Qword)
}

fn vpsllvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1450530800, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 71, 20, 221, 240, 87, 117, 86], OperandSize::Qword)
}

fn vpsllvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 71, 196], OperandSize::Dword)
}

fn vpsllvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 973676893, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 71, 190, 93, 33, 9, 58], OperandSize::Dword)
}

fn vpsllvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 71, 254], OperandSize::Qword)
}

fn vpsllvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 686053275, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 71, 12, 85, 155, 87, 228, 40], OperandSize::Qword)
}

fn vpsllvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 71, 209], OperandSize::Dword)
}

fn vpsllvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 71, 25], OperandSize::Dword)
}

fn vpsllvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 311983981, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 157, 71, 176, 109, 127, 152, 18], OperandSize::Dword)
}

fn vpsllvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 109, 141, 71, 234], OperandSize::Qword)
}

fn vpsllvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 132, 71, 35], OperandSize::Qword)
}

fn vpsllvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RDX, 47224543, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 5, 159, 71, 146, 223, 150, 208, 2], OperandSize::Qword)
}

fn vpsllvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 71, 237], OperandSize::Dword)
}

fn vpsllvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 71, 46], OperandSize::Dword)
}

fn vpsllvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 189, 71, 33], OperandSize::Dword)
}

fn vpsllvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 117, 174, 71, 218], OperandSize::Qword)
}

fn vpsllvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 127741017, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 117, 164, 71, 4, 77, 89, 44, 157, 7], OperandSize::Qword)
}

fn vpsllvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1913433823, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 45, 188, 71, 164, 217, 223, 174, 12, 114], OperandSize::Qword)
}

fn vpsllvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 71, 240], OperandSize::Dword)
}

fn vpsllvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 2049076873, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 71, 12, 189, 137, 110, 34, 122], OperandSize::Dword)
}

fn vpsllvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 304158536, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 223, 71, 143, 72, 23, 33, 18], OperandSize::Dword)
}

fn vpsllvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 29, 195, 71, 244], OperandSize::Qword)
}

fn vpsllvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 21, 196, 71, 36, 208], OperandSize::Qword)
}

fn vpsllvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RDI, 587252381, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 101, 220, 71, 135, 157, 194, 0, 35], OperandSize::Qword)
}

